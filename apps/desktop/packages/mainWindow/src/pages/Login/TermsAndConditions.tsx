import { Trans } from "@gd/i18n"
import { useModal } from "@/managers/ModalsManager"

const TermsAndConditions = () => {
  const modalsContext = useModal()

  return (
    <div class="flex-1 flex flex-col justify-between text-left gap-4 leading-5 p-4 text-lightSlate-700">
      <div class="flex flex-col gap-2">
        <p class="m-0 leading-5 text-sm select-none">
          <Trans key="login.read_and_accept">
            {""}
            <span
              class="cursor-pointer underline"
              onClick={() => {
                modalsContext?.openModal({
                  name: "termsAndConditions"
                })
              }}
            >
              {""}
            </span>
            {""}
            <span
              class="underline cursor-pointer"
              onClick={() => {
                modalsContext?.openModal({
                  name: "privacyStatement"
                })
              }}
            >
              {""}
            </span>
          </Trans>
        </p>
      </div>
    </div>
  )
}

export default TermsAndConditions
