import { Trans } from "@gd/i18n";
import { useModal } from "@/managers/ModalsManager";
import { useGlobalStore } from "@/components/GlobalStoreContext";
import { Show } from "solid-js";

const TermsAndConditions = () => {
  const modalsContext = useModal();
  const accountsLength = useGlobalStore().currentlySelectedAccount();

  return (
    <div class="flex-1 flex flex-col justify-between text-left gap-4 leading-5 p-4 text-lightSlate-900">
      <div class="flex flex-col gap-2 overflow-y-auto">
        <Show when={accountsLength}>
          <div>
            <Trans key="login.we_value_privacy_text_renew" />
          </div>
        </Show>
        <div>
          <Trans key="login.we_value_privacy_text1" />
        </div>
        <div>
          <Trans key="login.we_value_privacy_text2" />
        </div>
        <div>
          <Trans key="login.we_value_privacy_text3" />
        </div>
        <div>
          <Trans key="login.we_value_privacy_text4" />
        </div>
      </div>
      <div
        class="text-xs underline whitespace-nowrap text-lightSlate-50 cursor-pointer"
        onClick={() => {
          window?.openCMPWindow();
        }}
      >
        <Trans key="login.manage_cmp" />
      </div>

      <div class="flex flex-col gap-2">
        <p class="m-0 text-lightSlate-400 leading-5 text-xs select-none">
          <Trans key="login.read_and_accept">
            {""}
            <span
              class="cursor-pointer underline text-lightSlate-50"
              onClick={() => {
                modalsContext?.openModal({
                  name: "termsAndConditions"
                });
              }}
            >
              {""}
            </span>
            {""}
            <span
              class="underline text-lightSlate-50 cursor-pointer"
              onClick={() => {
                modalsContext?.openModal({
                  name: "privacyStatement"
                });
              }}
            >
              {""}
            </span>
          </Trans>
        </p>
      </div>
    </div>
  );
};

export default TermsAndConditions;
