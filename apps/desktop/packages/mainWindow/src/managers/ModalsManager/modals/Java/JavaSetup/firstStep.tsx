import { Trans } from "@gd/i18n"
import { StepsProps } from "."
import JavaLogo from "/assets/images/icons/java-logo.svg"
import { Button } from "@gd/ui"

const FirstStep = (props: StepsProps) => {
  return (
    <div class="w-110 h-75">
      <div class="flex flex-col justify-between w-full h-full">
        <div class="flex flex-col items-center">
          <img src={JavaLogo} class="h-16 w-16" />
          <h3 class="mb-0">
            <Trans
              key="java.java_missing"
              options={{
                defaultValue: "Java {{version}} missing",
                version: 8
              }}
            />
          </h3>
        </div>
        <p class="m-0 text-center text-darkSlate-300">
          <Trans
            key="java.missing_java_text"
            options={{
              defaultValue:
                "For an optimal experience, we sugges letting us take care of java for you. Only manually manage java if you know what yur're doing, it may result in GDLauncher not working!"
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
              props.nextStep?.("manual")
            }}
          >
            <Trans
              key="java.manual_setup"
              options={{
                defaultValue: "Manual setup"
              }}
            />
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
              key="java.automatic_setup"
              options={{
                defaultValue: "Automatic setup"
              }}
            />
          </Button>
        </div>
      </div>
    </div>
  )
}

export default FirstStep
