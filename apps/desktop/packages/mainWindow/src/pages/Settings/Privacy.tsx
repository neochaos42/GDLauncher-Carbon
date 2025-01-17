import { Button } from "@gd/ui"
import { Trans } from "@gd/i18n"
import PageTitle from "./components/PageTitle"
import RowsContainer from "./components/RowsContainer"
import Row from "./components/Row"
import Title from "./components/Title"
import RightHandSide from "./components/RightHandSide"
import { useModal } from "@/managers/ModalsManager"

const Privacy = () => {
  const modalsContext = useModal()

  return (
    <>
      <PageTitle>
        <Trans key="settings:Privacy" />
      </PageTitle>
      <RowsContainer>
        <Row>
          <Title
            description={<Trans key="settings:ads_personalization_text" />}
          >
            <Trans key="settings:ads_personalization_title" />
          </Title>
          <RightHandSide>
            <Button
              type="secondary"
              size="small"
              rounded={false}
              onClick={() => {
                window?.openCMPWindow()
              }}
            >
              <Trans key="login.manage" />
            </Button>
          </RightHandSide>
        </Row>
        <Row forceContentBelow>
          <Title description={<Trans key="settings:documents_text" />}>
            <Trans key="settings:documents_title" />
          </Title>
          <div class="flex gap-4">
            <Button
              type="secondary"
              size="small"
              rounded={false}
              onClick={() => {
                modalsContext?.openModal({
                  name: "privacyStatement"
                })
              }}
            >
              <Trans key="settings:privacy_policy" />
            </Button>

            <Button
              type="secondary"
              size="small"
              rounded={false}
              onClick={() => {
                modalsContext?.openModal({
                  name: "termsAndConditions"
                })
              }}
            >
              <Trans key="settings:terms_of_service" />
            </Button>
          </div>
        </Row>
      </RowsContainer>
    </>
  )
}

export default Privacy
