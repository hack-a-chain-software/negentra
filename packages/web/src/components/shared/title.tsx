/* eslint-disable @typescript-eslint/no-empty-interface */
import { Text as ChakraText, TextProps } from "@chakra-ui/react";

interface ITextProps extends TextProps {}

export function Title({
  fontSize = '100px',
  lineHeight = '114.5px',
  color = "#333333",
  fontFamily = 'Titan One',
  ...props
}: ITextProps) {
  return (
    <ChakraText
      color={color}
      fontSize={fontSize}
      lineHeight={lineHeight}
      fontFamily={fontFamily}
      {...props}
    />
  );
}
