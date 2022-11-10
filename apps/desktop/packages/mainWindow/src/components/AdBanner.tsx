import minimumBounds from "@/modules/components/minimumBounds";

export const AdsBanner = () => {
  return (
    <div
      style={{
        height: `${minimumBounds.adSize.height}px`,
        width: `${minimumBounds.adSize.width}px`,
      }}
      class="bg-red-400 mx-5 mt-5"
    />
  );
};