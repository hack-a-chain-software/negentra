/* eslint-disable @typescript-eslint/no-empty-interface */
import { Button as ChakraButton, ButtonProps, Flex } from "@chakra-ui/react";

interface IButtonProps extends ButtonProps {}

/**
 * @description - Chakra custom button component
 */
export function Button({
  bg = "linear-gradient(180deg, #D484F5 0%, #9B59B6 100%)",
  justifyContent = 'start',
  borderBottom = 'solid 8px #71368A',
  color = "white",
  _active = {
    borderBottomWidth: 0.4,
  },
  ...props
}: IButtonProps) {
  return (
    <ChakraButton
      bg={bg}
      color={color}
      justifyContent={justifyContent}
      borderBottom={borderBottom}
      borderRadius="18px"
      px="32px"
      py="16px"
      minHeight="72px"
      fontSize="18px"
      fontFamily="Titan One"
      alignItems="center"
      fontWeight="regular"
      lineHeight="21px"
      _active={_active}
      _hover={{
        // opacity: 0.4,
      }}
      {...props}
    />
  );
}
