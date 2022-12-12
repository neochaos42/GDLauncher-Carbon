import { useLocation, useNavigate } from "@solidjs/router";
import {
  Component,
  createEffect,
  createSignal,
  JSX,
  lazy,
  Show,
} from "solid-js";
import { createStore } from "solid-js/store";
import ModalLayout from "./ModalLayout";

/**
 * It renders a modal when the URL contains a query parameter called `m`
 * @returns A component that renders a modal.
 */

interface Hash {
  [name: string]: { component: JSX.Element; title: string };
}

const Modals: Component = () => {
  const location = useLocation();

  const navigate = useNavigate();
  const [isVisible, setIsVisible] = createSignal(false);
  const [opacity, setOpacity] = createSignal<0 | 1>(0);
  const [modals] = createStore<Hash>({
    privacyPolicy: {
      component: lazy(() => import("./modals/Privacypolicy")),
      title: "Privacy Policy",
    },
    termsAndConditions: {
      component: lazy(() => import("./modals/TermsAndConditions")),
      title: "Terms and Conditions",
    },
    acceptableUsePolicy: {
      component: lazy(() => import("./modals/AcceptableUsePolicy")),
      title: "Acceptable Use Policy",
    },
  });

  const queryParams = () => location.search;
  const mParam = () => new URLSearchParams(queryParams()).get("m");
  const isModal = () => mParam() !== null;

  const getModal = (type: string) => {
    const Component = () => modals[type]?.component;
    const title = () => modals[type]?.title;

    return (
      <ModalLayout onClose={() => navigate(location.pathname)} title={title()}>
        <Component />
      </ModalLayout>
    );
  };

  createEffect(() => {
    const visibility = isModal();
    // When the URL changes, update the visibility of the modal after a timeout
    if (visibility) {
      setIsVisible(visibility);
      setTimeout(() => {
        setOpacity(1);
      }, 20);
    } else {
      setOpacity(0);
      setTimeout(() => {
        setIsVisible(visibility);
      }, 150);
    }
  });

  return (
    <div
      class="absolute opacity-0 scale-0 will-change-auto transition-opacity w-screen h-screen backdrop-blur-sm backdrop-brightness-50 grid place-items-center text-white z-999"
      classList={{
        "opacity-100": !!opacity(),
        "scale-100": !!opacity(),
      }}
      onClick={() => {
        navigate(location.pathname);
      }}
    >
      <Show when={isVisible()}>
        <div
          onClick={(e) => {
            e.stopPropagation();
          }}
        >
          {getModal(mParam() || "")}
        </div>
      </Show>
    </div>
  );
};

export default Modals;