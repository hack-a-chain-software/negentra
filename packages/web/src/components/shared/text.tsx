/* eslint-disable @typescript-eslint/no-empty-interface */
import { Text as ChakraText, TextProps } from "@chakra-ui/react";

interface ITextProps extends TextProps {}

export function Text({
  fontSize = '18px',
  lineHeight = '24.51px',
  color = "#333333",
  fontWeight = 400,
  ...props
}: ITextProps) {
  return (
    <ChakraText
      color={color}
      fontSize={fontSize}
      fontWeight={fontWeight}
      lineHeight={lineHeight}
      {...props}
    />
  );
}
