import { Trans } from "@gd/i18n";
import { useGlobalStore } from "@/components/GlobalStoreContext";
import { Show } from "solid-js";

const PrivacyNotice = () => {
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
          <Trans key="login.we_value_privacy_text2">
            {""}
            <span
              class="underline"
              onClick={() => {
                window?.openCMPWindow();
              }}
            >
              {""}
            </span>
            {""}
          </Trans>
        </div>
        <div>
          <Trans key="login.we_value_privacy_text3" />
        </div>
        <div>
          <Trans key="login.we_value_privacy_text4" />
        </div>
        <div>
          <Trans key="login.we_value_privacy_text5" />
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
    </div>
  );
};

export default PrivacyNotice;
