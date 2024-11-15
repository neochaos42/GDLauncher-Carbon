import { convertSecondsToHumanTime } from "@/utils/helpers";
import { rspc } from "@/utils/rspcClient";
import { Trans } from "@gd/i18n";
import { Navigate } from "@solidjs/router";
import { createSignal, Match, Switch } from "solid-js";

interface Props {
  nextStep: () => void;
  prevStep: () => void;
  activeUuid: string | null | undefined;
  transitionToLibrary: () => void;
}

const GDLAccountVerification = (props: Props) => {
  const [cooldown, setCooldown] = createSignal(0);
  const [sentVisible, setSentVisible] = createSignal(false);

  const saveGdlAccountMutation = rspc.createMutation(() => ({
    mutationKey: ["account.saveGdlAccount"]
  }));

  const peekedUser = rspc.createQuery(() => ({
    queryKey: ["account.peekGdlAccount", props.activeUuid!],
    enabled: !!props.activeUuid
  }));

  const verified = rspc.createQuery(() => ({
    queryKey: ["account.waitForAccountValidation", props.activeUuid!],
    enabled: !!props.activeUuid
  }));

  const requestNewVerificationTokenMutation = rspc.createMutation(() => ({
    mutationKey: ["account.requestNewVerificationToken"]
  }));

  let cooldownInterval: ReturnType<typeof setInterval> | undefined;

  return (
    <>
      <Switch>
        <Match when={verified.isSuccess}>
          <Navigate href="/library" />
        </Match>
        <Match when={!verified.isSuccess}>
          <div class="flex-1 w-full text-center gap-5 flex flex-col justify-between items-center">
            <div class="p-10">
              <div class="text-2xl font-bold">
                <Trans key="login.check_your_email_for_a_verification_link" />
              </div>
              <div class="pt-4 pb-10 text-lightSlate-600">
                ({peekedUser.data?.email})
              </div>
              <div
                onClick={async () => {
                  if (cooldownInterval) {
                    return;
                  }

                  if (!props.activeUuid) {
                    throw new Error("No active uuid");
                  }

                  try {
                    const status =
                      await requestNewVerificationTokenMutation.mutateAsync(
                        props.activeUuid
                      );

                    if (status.status === "failed" && status.value) {
                      setSentVisible(false);

                      clearInterval(cooldownInterval);
                      cooldownInterval = undefined;

                      setCooldown(status.value);

                      cooldownInterval = setInterval(() => {
                        setCooldown((prev) => prev - 1);

                        if (cooldown() <= 0) {
                          setCooldown(0);
                          clearInterval(cooldownInterval);
                          cooldownInterval = undefined;
                        }
                      }, 1000);
                    } else if (status.status === "success") {
                      setSentVisible(true);
                      setTimeout(() => setSentVisible(false), 10000);
                    }
                  } catch (e) {
                    console.error(e);
                  }
                }}
                class="underline transition-all duration-100 ease-in-out"
                classList={{
                  "text-lightSlate-400 hover:text-lightSlate-50": !cooldown(),
                  "text-lightSlate-900": !!cooldown()
                }}
              >
                <Trans key="login.request_a_new_verification_link" />
              </div>
              <div class="text-sm mt-2">
                <Switch>
                  <Match when={sentVisible()}>
                    <div class="text-green-500">
                      <Trans key="login.an_email_has_been_sent_to_your_email_address" />
                    </div>
                  </Match>
                  <Match when={cooldown()}>
                    <div class="text-lightSlate-500">
                      <Trans
                        key="login.email_request_wait"
                        options={{
                          time: convertSecondsToHumanTime(cooldown())
                        }}
                      />
                    </div>
                  </Match>
                </Switch>
              </div>
            </div>

            <div
              onClick={async () => {
                await props.transitionToLibrary?.();
                await saveGdlAccountMutation.mutateAsync(props.activeUuid!);
              }}
              class="underline text-lightSlate-400 hover:text-lightSlate-50 transition-all duration-100 ease-in-out"
            >
              <Trans key="login.verify_later" />
            </div>
          </div>
        </Match>
      </Switch>
    </>
  );
};

export default GDLAccountVerification;
