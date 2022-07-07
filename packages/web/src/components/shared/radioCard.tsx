import { Box, useRadio } from '@chakra-ui/react';

export function RadioCard(props) {
  const { getInputProps, getCheckboxProps } = useRadio(props)

  const input = getInputProps()
  const checkbox = getCheckboxProps()

  return (
    <Box 
      as='label'
    >
      <input 
        {...input} 
      />
      <Box
        {...checkbox}
        cursor='pointer'
        borderRadius="18px"
        minWidth="170px"
        border="solid 1px white"
        padding="6px 12px"
        background="linear-gradient(180deg, #D484F5 0%, #9B59B6 100%)"
        _checked={{
          background: '#FF6F00',
        }}
        outline="none"
      >
        {props.children}
      </Box>
    </Box>
  )
};
