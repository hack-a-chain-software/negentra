import { Text, Flex, Image } from '@chakra-ui/react';

export function PersonCard(props) {
  return (
    <Flex
      width="242px"
      height="343px"
      direction="column"
      overflow="hidden"
      position="relative"
    >
      <Image
        left="0"
        zIndex="0"
        position="absolute"
        h="100%"
        w="100%"
        src="/svg/person-card-bg.svg"
      />

      <Flex
        zIndex="1"
        position="relative"
        paddingTop="45px"
        paddingLeft="50px"
      >
        <Image
          src={props.image}
        />
      </Flex>

      <Flex
        zIndex="1"
        direction="column"
        position="relative"
        marginTop="auto"
        paddingBottom="30px"
      >
        <Text
          fontSize="18px"
          color="#FFFFFF"
          textAlign="center"
          fontFamily="Titan One"
          transform="rotate(-3deg)"
        >
          { props.name }
        </Text>

        <Text
          fontSize="16px"
          color="#FFFFFF"
          textAlign="center"
          transform="rotate(-3deg)"
        >
          { props.role }
        </Text>
      </Flex>
    </Flex>
  );
};