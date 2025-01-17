import { Button, createNotification } from "@gd/ui"
import {
  createSignal,
  Switch,
  Match,
  createEffect,
  onMount,
  For,
  Show
} from "solid-js"
import Auth from "./Auth"
import CodeStep from "./CodeStep"
import fetchData from "./auth.login.data"
import { useRouteData, useSearchParams } from "@solidjs/router"
import { Trans } from "@gd/i18n"
import { rspc } from "@/utils/rspcClient"
import TermsAndConditions from "./TermsAndConditions"
import Logo from "/assets/images/gdlauncher_wide_logo_blue.svg"
import BackgroundVideo from "/assets/images/login_background.webm"
import { handleStatus } from "@/utils/login"
import { parseError } from "@/utils/helpers"
import GDLAccount from "./GDLAccount"
import GDLAccountCompletion from "./GDLAccountCompletion"
import { useGDNavigate } from "@/managers/NavigationManager"
import GDLAccountVerification from "./GDLAccountVerification"
import { useGlobalStore } from "@/components/GlobalStoreContext"
import PrivacyNotice from "./PrivacyNotice"

export interface DeviceCodeObjectType {
  userCode: string
  link: string
  expiresAt: string
}

enum Steps {
  TermsAndConditions = 1,
  PrivacyNotice = 2,
  Auth = 3,
  CodeStep = 4,
  GDLAccount = 5,
  GDLAccountCompletion = 6,
  GDLAccountVerification = 7
}

export default function Login() {
  const routeData: ReturnType<typeof fetchData> = useRouteData()
  const [searchParams] = useSearchParams()

  const globalStore = useGlobalStore()

  const navigate = useGDNavigate()
  const [step, setStep] = createSignal<Steps>(Steps.TermsAndConditions)
  const [deviceCodeObject, setDeviceCodeObject] =
    createSignal<DeviceCodeObjectType | null>(null)
  const [loadingButton, setLoadingButton] = createSignal(false)
  const activeUuid = globalStore.currentlySelectedAccountUuid

  const gdlUser = rspc.createQuery(() => ({
    queryKey: ["account.peekGdlAccount", activeUuid.data!],
    enabled: !!activeUuid.data
  }))

  const saveGdlAccountMutation = rspc.createMutation(() => ({
    mutationKey: ["account.saveGdlAccount"]
  }))

  createEffect((prev) => {
    if (activeUuid.data && activeUuid.data !== prev) {
      gdlUser.refetch()
    }

    return activeUuid.data
  })

  const [recoveryEmail, setRecoveryEmail] = createSignal<string | null>(null)
  const [nickname, setNickname] = createSignal<string | null>(null)

  const [acceptedHashedEmail, setAcceptedHashedEmail] = createSignal(
    globalStore.settings.data?.hashedEmailAccepted
  )

  const [cooldown, setCooldown] = createSignal(0)

  let cooldownInterval: ReturnType<typeof setInterval> | undefined

  const rspcContext = rspc.useContext()

  const settingsMutation = rspc.createMutation(() => ({
    mutationKey: ["settings.setSettings"]
  }))

  const deleteGDLAccountMutation = rspc.createMutation(() => ({
    mutationKey: ["account.removeGdlAccount"]
  }))

  const requestEmailChangeMutation = rspc.createMutation(() => ({
    mutationKey: ["account.requestEmailChange"]
  }))

  const registerGdlAccountMutation = rspc.createMutation(() => ({
    mutationKey: ["account.registerGdlAccount"]
  }))

  const changeGDLAccountNicknameMutation = rspc.createMutation(() => ({
    mutationKey: ["account.changeGdlAccountNickname"]
  }))

  const [isBackButtonVisible, setIsBackButtonVisible] = createSignal(false)

  const isGDLAccountSet = () =>
    globalStore.settings.data?.gdlAccountId ||
    globalStore.settings.data?.gdlAccountId === ""

  const accountEnrollFinalizeMutation = rspc.createMutation(() => ({
    mutationKey: ["account.enroll.finalize"]
  }))

  const nextStep = () => {
    if (step() < Steps.GDLAccountVerification) {
      setStep((prev) => prev + 1)
    }
  }

  const prevStep = () => {
    if (step() >= Steps.TermsAndConditions) {
      setStep((prev) => prev - 1)
    }
  }

  let sidebarRef: HTMLDivElement | undefined
  let backgroundBlurRef: HTMLDivElement | undefined
  let welcomeToTextRef: HTMLDivElement | undefined
  let gdlauncherTextRef: HTMLDivElement | undefined
  let videoRef: HTMLVideoElement | undefined

  async function transitionToLibrary() {
    return new Promise((resolve) => {
      if (backgroundBlurRef && globalStore.settings.data?.isFirstLaunch) {
        sidebarRef?.animate(
          [{ transform: "translateX(0%)" }, { transform: "translateX(-100%)" }],
          {
            duration: 500,
            easing: "linear",
            fill: "forwards"
          }
        )

        videoRef?.animate(
          [{ transform: "translateX(15%)" }, { transform: "translateX(0%)" }],
          {
            duration: 300,
            easing: "linear",
            fill: "forwards"
          }
        )

        backgroundBlurRef.animate([{ opacity: 0 }, { opacity: 1 }], {
          duration: 500,
          delay: 350,
          easing: "linear",
          fill: "forwards"
        })

        welcomeToTextRef?.animate([{ opacity: 0 }, { opacity: 1 }], {
          duration: 600,
          delay: 1100,
          easing: "linear",
          fill: "forwards"
        })

        gdlauncherTextRef?.animate([{ opacity: 0 }, { opacity: 1 }], {
          duration: 600,
          delay: 2300,
          easing: "linear",
          fill: "forwards"
        })

        setTimeout(() => {
          navigate("/library")
          resolve(null)
        }, 5000)
      } else {
        navigate("/library")
        resolve(null)
      }
    })
  }

  const addNotification = createNotification()

  createEffect(() => {
    handleStatus(routeData.status, {
      onPolling: async (info) => {
        setDeviceCodeObject({
          userCode: info.userCode,
          link: info.verificationUri,
          expiresAt: info.expiresAt
        })
        if (routeData.status.data) {
          setStep(Steps.CodeStep)
          setLoadingButton(false)
        }
      },
      async onError(error) {
        if (error)
          addNotification({
            name: parseError(error),
            type: "error"
          })
        setStep(Steps.Auth)
        setLoadingButton(false)
      },
      async onComplete() {
        await accountEnrollFinalizeMutation.mutateAsync(undefined)

        const activeUuid = await rspcContext.client.query([
          "account.getActiveUuid"
        ])

        if (!activeUuid) {
          throw new Error("No active uuid")
        }

        const settings = await rspcContext.client.query([
          "settings.getSettings"
        ])

        // Already has GDL account
        if (
          settings.gdlAccountId !== null &&
          settings.gdlAccountId !== undefined
        ) {
          transitionToLibrary()
          return
        }

        const gdlUserPeek = await rspcContext.client.query([
          "account.peekGdlAccount",
          activeUuid
        ])

        if (gdlUserPeek?.email) {
          setRecoveryEmail(gdlUserPeek.email)
          setNickname(gdlUserPeek.nickname)
        }

        setStep(Steps.GDLAccount)
        setLoadingButton(false)
      }
    })
  })

  onMount(async () => {
    requestAnimationFrame(() => {
      handleSidebarAnimation()
    })

    const activeUuid = await rspcContext.client.query(["account.getActiveUuid"])

    if (!globalStore.settings.data?.termsAndPrivacyAccepted) {
      setStep(Steps.TermsAndConditions)
      setIsBackButtonVisible(false)
      return
    }

    const accounts = await rspcContext.client.query(["account.getAccounts"])

    if (
      !activeUuid ||
      accounts.length === 0 ||
      searchParams.addMicrosoftAccount
    ) {
      setStep(Steps.Auth)
      setIsBackButtonVisible(true)
      return
    }

    if (isGDLAccountSet()) {
      transitionToLibrary()
    } else if (!isGDLAccountSet()) {
      setIsBackButtonVisible(true)
      setStep(Steps.GDLAccount)
    }

    return
  })

  async function handleSidebarAnimation() {
    if (sidebarRef) {
      await new Promise((resolve) => setTimeout(resolve, 300))

      sidebarRef.animate(
        [{ transform: "translateX(-100%)" }, { transform: "translateX(0)" }],
        {
          duration: 300,
          delay: 200,
          easing: "cubic-bezier(0.175, 0.885, 0.32, 1)",
          fill: "forwards"
        }
      )

      videoRef?.animate(
        [{ transform: "translateX(0)" }, { transform: "translateX(15%)" }],
        {
          duration: 300,
          delay: 200,
          easing: "cubic-bezier(0.175, 0.885, 0.32, 1)",
          fill: "forwards"
        }
      )
    }
  }

  let btnRef: HTMLDivElement | undefined

  function handleAnimationForward() {
    if (btnRef) {
      if (isBackButtonVisible()) return

      setIsBackButtonVisible(true)
      btnRef.animate(
        [
          { width: "0", margin: "0" },
          { width: "60%", margin: "0 1rem 0 0" }
        ],
        {
          duration: 300,
          easing: "cubic-bezier(0.175, 0.885, 0.32, 1.275)",
          fill: "forwards"
        }
      )
    }
  }

  function handleAnimationBackward() {
    if (btnRef && isBackButtonVisible() && step() === Steps.PrivacyNotice) {
      setIsBackButtonVisible(false)

      btnRef.animate(
        [
          { width: "60%", margin: "0 1rem 0 0" },
          { width: "0", margin: "0" }
        ],
        {
          duration: 300,
          easing: "cubic-bezier(0.175, 0.885, 0.32, 1.275)",
          fill: "forwards"
        }
      )
    }
  }

  const accountEnrollCancelMutation = rspc.createMutation(() => ({
    mutationKey: ["account.enroll.cancel"]
  }))

  const accountEnrollBeginMutation = rspc.createMutation(() => ({
    mutationKey: ["account.enroll.begin"],

    onError() {
      retryLogin()
    }
  }))

  const [retry, setRetry] = createSignal(0)

  const retryLogin = () => {
    while (retry() <= 3) {
      if (!routeData.status.data) {
        accountEnrollCancelMutation.mutate(undefined)
      }
      accountEnrollBeginMutation.mutate(undefined)
      setRetry((prev) => prev + 1)
    }
    if (retry() > 3) {
      addNotification({
        name: "Something went wrong while logging in, try again!",
        type: "error"
      })
      if (routeData.status.data) {
        accountEnrollCancelMutation.mutate(undefined)
      }
    }
  }

  return (
    <div class="flex w-full h-screen" id="main-login-page">
      <div
        ref={sidebarRef}
        class="z-10 absolute -translate-x-full w-100 h-full flex flex-col items-center text-lightSlate-50 rounded-md bg-darkSlate-800 z-1"
      >
        <div class="flex justify-center h-30">
          <img class="w-60" src={Logo} />
        </div>
        <div class="text-lg font-bold flex items-center justify-center gap-2 mb-4">
          <Switch>
            <Match when={step() === Steps.TermsAndConditions}>
              <div>
                <div>
                  <Trans key="login.titles.welcome_to_gdlauncher" />
                </div>
                <Show when={activeUuid?.data}>
                  <div>
                    <Trans key="login.renew" />
                  </div>
                </Show>
              </div>
            </Match>
            <Match when={step() === Steps.PrivacyNotice}>
              <Trans key="login.titles.we_value_privacy" />
            </Match>
            <Match when={step() === Steps.Auth}>
              <Trans key="login.titles.sign_in_with_microsoft" />
            </Match>
            <Match when={step() === Steps.CodeStep}>
              <i class="inline-block w-4 h-4 i-ri:microsoft-fill" />
              <Trans key="login.titles.microsoft_code_step" />
            </Match>
            <Match when={step() === Steps.GDLAccount}>
              <Switch>
                <Match when={gdlUser.data}>
                  <Trans key="login.titles.sync_gdl_account" />
                </Match>
                <Match when={!gdlUser.data}>
                  <Trans key="login.titles.create_gdl_account" />
                </Match>
              </Switch>
            </Match>
            <Match when={step() === Steps.GDLAccountCompletion}>
              <Trans key="login.titles.linked_microsoft_account" />
            </Match>
            <Match when={step() === Steps.GDLAccountVerification}>
              <Trans key="login.titles.gdl_account_verification" />
            </Match>
          </Switch>
        </div>
        <div class="flex flex-1 w-full h-auto overflow-y-auto px-4 box-border">
          <Switch>
            <Match when={step() === Steps.TermsAndConditions}>
              <TermsAndConditions />
            </Match>
            <Match when={step() === Steps.PrivacyNotice}>
              <PrivacyNotice />
            </Match>
            <Match when={step() === Steps.Auth}>
              <Auth />
            </Match>
            <Match when={step() === Steps.CodeStep}>
              <CodeStep
                nextStep={nextStep}
                prevStep={prevStep}
                deviceCodeObject={deviceCodeObject()}
                setDeviceCodeObject={setDeviceCodeObject}
              />
            </Match>
            <Match when={step() === Steps.GDLAccount}>
              <GDLAccount activeUuid={activeUuid.data} />
            </Match>
            <Match when={step() === Steps.GDLAccountCompletion}>
              <GDLAccountCompletion
                nextStep={nextStep}
                prevStep={prevStep}
                recoveryEmail={recoveryEmail()}
                setRecoveryEmail={setRecoveryEmail}
                nickname={nickname()}
                setNickname={setNickname}
                cooldown={cooldown()}
                acceptedHashedEmail={!!acceptedHashedEmail()}
                setAcceptedHashedEmail={setAcceptedHashedEmail}
              />
            </Match>
            <Match when={step() === Steps.GDLAccountVerification}>
              <GDLAccountVerification
                nextStep={nextStep}
                prevStep={prevStep}
                activeUuid={activeUuid.data}
                transitionToLibrary={transitionToLibrary}
              />
            </Match>
          </Switch>
        </div>

        <div class="w-full flex flex-col items-center p-4 box-border">
          <div class="relative flex justify-center gap-2 mb-4">
            <div class="absolute top-1/2 left-0 -translate-y-1/2 h-4 w-full rounded-lg overflow-hidden">
              <div
                class="absolute top-0 left-0 bg-darkSlate-400 h-4 w-full rounded-lg"
                style={{
                  transform: `translateX(calc((-100% + ${(100 * step()) / 7}%) - ${(step() === Steps.TermsAndConditions ? 9 : 7) - step()}px)`,
                  transition:
                    "transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275)"
                }}
              />
            </div>

            <For each={new Array(7)}>
              {(_, i) => (
                <div
                  class="z-1 h-6 w-4 flex justify-center items-center group"
                  onClick={() => {
                    if (
                      i() + 1 < step() &&
                      (step() > Steps.CodeStep
                        ? i() + 1 > Steps.CodeStep
                        : step() >= Steps.Auth &&
                            searchParams.addMicrosoftAccount
                          ? false
                          : true)
                    ) {
                      setLoadingButton(false)
                      setStep(i() + 1)
                    }
                  }}
                >
                  <div
                    class="h-2 w-2 bg-lightSlate-900 rounded-full"
                    classList={{
                      "group-hover:bg-lightSlate-100":
                        i() + 1 < step() &&
                        (step() > Steps.CodeStep
                          ? i() + 1 > Steps.CodeStep
                          : step() >= Steps.Auth &&
                              searchParams.addMicrosoftAccount
                            ? false
                            : true)
                    }}
                  />
                </div>
              )}
            </For>
          </div>

          <div class="flex w-full box-border">
            <div
              ref={btnRef}
              class="overflow-hidden"
              style={{
                width: !isBackButtonVisible() ? "0" : "60%",
                margin: !isBackButtonVisible() ? "0" : "0 1rem 0 0"
              }}
            >
              <Button
                size="large"
                type="secondary"
                fullWidth
                onClick={async () => {
                  if (step() === Steps.GDLAccount) {
                    await saveGdlAccountMutation.mutateAsync("")
                    transitionToLibrary()
                  } else if (
                    (step() === Steps.Auth || step() === Steps.CodeStep) &&
                    globalStore.accounts.data?.length !== 0
                  ) {
                    navigate("/settings/accounts")
                  } else {
                    handleAnimationBackward()
                    prevStep()
                  }

                  setLoadingButton(false)
                }}
              >
                <Switch>
                  <Match
                    when={
                      step() === Steps.GDLAccount ||
                      ((step() === Steps.Auth || step() === Steps.CodeStep) &&
                        globalStore.accounts.data?.length !== 0)
                    }
                  >
                    <Trans key="general.skip" />
                    <i class="i-ri:skip-forward-line" />
                  </Match>
                  <Match when={step() !== Steps.GDLAccount}>
                    <i class="i-ri:arrow-left-line" />
                    <Trans key="general.back" />
                  </Match>
                </Switch>
              </Button>
            </div>
            <Button
              fullWidth
              variant="primary"
              size="large"
              disabled={
                step() === Steps.CodeStep ||
                step() === Steps.GDLAccountVerification ||
                (step() === Steps.GDLAccountCompletion &&
                  (!recoveryEmail() || !nickname()))
              }
              loading={
                loadingButton() || step() === Steps.GDLAccountVerification
              }
              onClick={async () => {
                handleAnimationForward()
                setLoadingButton(true)

                if (step() === Steps.TermsAndConditions) {
                  try {
                    await settingsMutation.mutateAsync({
                      termsAndPrivacyAccepted: {
                        Set: true
                      },
                      hashedEmailAccepted: {
                        Set: !!acceptedHashedEmail()
                      }
                    })
                  } catch (err) {
                    console.log(err)
                    addNotification({
                      name: "Error while accepting terms and conditions",
                      content: "Check the console for more information.",
                      type: "error"
                    })
                  }

                  setLoadingButton(false)

                  if (!searchParams.addMicrosoftAccount && activeUuid.data) {
                    navigate("/library")
                  } else {
                    nextStep()
                  }
                } else if (step() === Steps.PrivacyNotice) {
                  setLoadingButton(false)
                  nextStep()
                } else if (step() === Steps.Auth) {
                  if (!routeData.status.data) {
                    await accountEnrollBeginMutation.mutateAsync(undefined)
                  } else {
                    await accountEnrollCancelMutation.mutateAsync(undefined)
                    await accountEnrollBeginMutation.mutateAsync(undefined)
                  }
                } else if (step() === Steps.GDLAccount) {
                  const uuid = globalStore?.currentlySelectedAccountUuid?.data

                  if (!uuid) {
                    throw new Error("No active uuid")
                  }

                  try {
                    const existingGDLUser = await rspcContext.client.query([
                      "account.peekGdlAccount",
                      uuid
                    ])

                    if (existingGDLUser?.isEmailVerified) {
                      transitionToLibrary()
                      await saveGdlAccountMutation.mutateAsync(uuid)

                      return
                    } else if (
                      existingGDLUser &&
                      !existingGDLUser.isEmailVerified
                    ) {
                      setRecoveryEmail(existingGDLUser.email)
                      setNickname(existingGDLUser.nickname)
                      setStep(Steps.GDLAccountVerification)
                      return
                    }
                  } catch (e) {
                    console.error(e)
                  }

                  await deleteGDLAccountMutation.mutateAsync(undefined)
                  setLoadingButton(false)
                  nextStep()
                } else if (step() === Steps.GDLAccountCompletion) {
                  const uuid = globalStore?.currentlySelectedAccountUuid?.data

                  if (!uuid) {
                    throw new Error("No active uuid")
                  }

                  const email = recoveryEmail()

                  if (!email) {
                    throw new Error("No recovery email")
                  }

                  try {
                    const existingGDLUser = await rspcContext.client.query([
                      "account.peekGdlAccount",
                      uuid
                    ])

                    if (
                      existingGDLUser?.nickname &&
                      existingGDLUser.nickname !== nickname()
                    ) {
                      try {
                        await changeGDLAccountNicknameMutation.mutateAsync({
                          uuid,
                          nickname: nickname()!
                        })
                      } catch (e) {
                        console.error(e)
                        setLoadingButton(false)
                        return
                      }
                    }

                    if (
                      existingGDLUser?.email &&
                      existingGDLUser.email !== recoveryEmail()
                    ) {
                      try {
                        const result =
                          await requestEmailChangeMutation.mutateAsync({
                            uuid,
                            email: recoveryEmail()!
                          })

                        if (result.status === "success") {
                          setStep(Steps.GDLAccountVerification)
                          setLoadingButton(false)
                        } else if (result.status === "failed" && result.value) {
                          clearInterval(cooldownInterval)
                          cooldownInterval = undefined

                          setLoadingButton(false)
                          setCooldown(result.value)
                          setRecoveryEmail(existingGDLUser.email)

                          cooldownInterval = setInterval(() => {
                            setCooldown((prev) => prev - 1)

                            if (cooldown() <= 0) {
                              setCooldown(0)
                              clearInterval(cooldownInterval)
                              cooldownInterval = undefined
                            }
                          }, 1000)
                        }
                      } catch (e) {
                        console.error(e)
                        addNotification({
                          name: "Error while requesting email change",
                          content: (e as any).message,
                          type: "error"
                        })
                      }
                    } else if (existingGDLUser?.isEmailVerified) {
                      transitionToLibrary()
                    } else if (
                      existingGDLUser &&
                      !existingGDLUser.isEmailVerified
                    ) {
                      setRecoveryEmail(existingGDLUser.email)
                      setNickname(existingGDLUser.nickname)
                      setStep(Steps.GDLAccountVerification)
                    } else {
                      await registerGdlAccountMutation.mutateAsync({
                        email: recoveryEmail()!,
                        nickname: nickname()!,
                        uuid
                      })

                      await saveGdlAccountMutation.mutateAsync(uuid)

                      nextStep()
                    }
                  } catch (e) {
                    setLoadingButton(false)
                    console.error(e)
                  }
                }
              }}
            >
              <Switch>
                <Match when={step() === Steps.TermsAndConditions}>
                  <Trans key="login.agree_and_continue" />
                  <i class="i-ri:arrow-right-line" />
                </Match>
                <Match when={step() === Steps.PrivacyNotice}>
                  <Trans key="login.accept_all_and_continue" />
                  <i class="i-ri:arrow-right-line" />
                </Match>
                <Match
                  when={step() === Steps.GDLAccountCompletion && !gdlUser.data}
                >
                  <Trans key="login.register" />
                  <i class="i-ri:arrow-right-line" />
                </Match>
                <Match
                  when={
                    step() === Steps.GDLAccountCompletion &&
                    gdlUser.data &&
                    gdlUser.data.email !== recoveryEmail()
                  }
                >
                  <Trans key="login.request_email_change" />
                  <i class="i-ri:arrow-right-line" />
                </Match>
                <Match when={step() === Steps.GDLAccount && gdlUser.data}>
                  <Trans key="login.sync_gdl_account" />
                  <i class="i-ri:arrow-right-line" />
                </Match>
                <Match when={step() === Steps.Auth}>
                  <i class="w-4 h-4 i-ri:microsoft-fill" />
                  <Trans key="login.sign_in" />
                </Match>
                <Match
                  when={step() === Steps.TermsAndConditions && activeUuid.data}
                >
                  <Trans key="instance_confirm" />
                </Match>
                <Match when={step() !== Steps.Auth}>
                  <Trans key="login.next" />
                  <i class="i-ri:arrow-right-line" />
                </Match>
              </Switch>
            </Button>
          </div>
        </div>
      </div>
      <div class="flex-1 w-full">
        <div
          ref={backgroundBlurRef}
          class="z-1 absolute top-0 left-0 p-0 h-screen w-full opacity-0 bg-black/20"
          style={{
            "backdrop-filter": "blur(6px)"
          }}
        />
        <div class="z-1 font-bold text-7xl leading-loose absolute top-0 left-0 p-0 h-screen w-full flex flex-col items-center justify-center">
          <div ref={welcomeToTextRef} class="opacity-0">
            <Trans key="login.welcome_to" />
          </div>
          <div ref={gdlauncherTextRef} class="opacity-0">
            <Trans key="login.gdlauncher" />
          </div>
        </div>
        <video
          ref={videoRef}
          class="p-0 h-screen w-full object-cover"
          src={BackgroundVideo}
          autoplay
          muted
          loop
          playsinline
        />
        {/* <div
              style={{
                "mix-blend-mode": "hard-light"
              }}
              class="absolute left-0 right-0 bg-darkSlate-800 bottom-0 top-0 opacity-30"
            /> */}
      </div>
    </div>
  )
}
