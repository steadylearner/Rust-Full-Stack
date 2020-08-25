import * as React from "react";

export const MediaComponent = ({ blockProps }: any) => {
    const src = blockProps.src;
    if (src.file) {
        return (
            <img
                style={{
                    width: "100%"
                }}
                src={src.file}
                alt={`Image(${src.file})`}
            />
        );
    }
    return null;
};