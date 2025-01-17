import { Button } from "@gd/ui"
import { StepsProps } from "."
import { Trans } from "@gd/i18n"

const ManualStep = (props: StepsProps) => {
  return (
    <div class="w-110 h-65">
      <div class="flex flex-col justify-between w-full h-full">
        <div class="flex flex-col justify-center items-center h-13 py-4 border-dashed border-2 border-primary-500">
          <div class="flex flex-col justify-center items-center gap-2">
            <div class="text-lightSlate-700 text-xl w-6 i-ri:folder-open-fill" />
            <p class="m-0 text-lightSlate-700">
              <Trans
                key="java.select_java_zip"
                options={{
                  defaultValue: "Select java {{version}} zip",
                  version: 8
                }}
              />
            </p>
          </div>
        </div>
        <p class="text-lightSlate-700 text-center">
          <Trans
            key="java.select_required_java_text"
            options={{
              defaultValue:
                "Select the required paths to java. Java 8 is used for all the versions < 1.17"
            }}
          />
        </p>
        <div class="w-full flex justify-between gap-4">
          <Button
            rounded
            type="secondary"
            size="large"
            style={{ width: "100%", "max-width": "100%" }}
            onClick={() => {
              props.nextStep?.("intro")
            }}
          >
            <Trans key="java.step_back" />
          </Button>
          <Button
            rounded
            size="large"
            style={{ width: "100%", "max-width": "100%" }}
            onClick={() => {
              props.nextStep?.("automatic")
            }}
          >
            <Trans
              key="java.setup"
              options={{
                defaultValue: "Setup"
              }}
            />
          </Button>
        </div>
      </div>
    </div>
  )
}

export default ManualStep
