/* eslint-disable @typescript-eslint/no-empty-interface */
import { Button as ChakraButton, ButtonProps, Flex } from "@chakra-ui/react";

interface IButtonProps extends ButtonProps {}

/**
 * @description - Chakra custom button component
 */
export function Button3d({
  width = 'auto',
  height = 'auto',
  borderRadius = '18px',
  bg =  '#71368A',
  minHeight = '72px',
  padding = '16px 32px',
  color = "linear-gradient(180deg, #D484F5 0%, #9B59B6 100%)",
  ...props
}: IButtonProps) {
  return (
    <ChakraButton
      bg={bg}
      width="max"
      padding="0px"
      minHeight={minHeight}
      borderRadius={borderRadius}
      _hover={{
        //
      }}
      _active={{
        //
      }}
      {...props}
    >
      <Flex
        bg={color}
        color="white"
        width={width}
        height={height}
        minHeight={minHeight}
        borderRadius={borderRadius}
        padding={padding}
        transform="translateY(-6px)"
        _active={{
          transform: 'translateY(-2px)',
        }}
      >
        { props.children }
      </Flex>
    </ChakraButton>
  );
}
