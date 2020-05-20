import * as React from "react";
//eslint-disable-next-line
export const MediaComponent = ({ blockProps }: any) => {
  const src = blockProps.src;
  if (src.file) {
    return (
      <img
        style={{
          width: "100%"
        }}
        src={src.file}
        alt="article"
      />
    );
  }
  return null;
};
