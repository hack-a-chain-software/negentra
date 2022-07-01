import { Container, Flex, Image } from '@chakra-ui/react';
import { Button, Text, Title } from '@negentra/src/components';

export function FollowUp() {
  return (
    <Container
      w="100vw"
      maxWidth="1410px"
      marginBottom="200px"
      className="flex items-center flex-col mb-[142px]"
    >
      <Flex
        width="1091px"
        height="640px"
        direction="column"
        paddingTop="101px"
        paddingLeft="112px"
        position="relative"
        className="bg-[url(/svg/follow-up-bg.svg)] bg-[center_top_-88px]"
      >
        <Flex
          maxWidth="600px"
          marginBottom="20px"
        >
          <Title
            color="white"
            fontSize="60px"
            lineHeight="70px"
          >
            CONNECT {'&'} FOLLOW SOCIAPOL!
          </Title>
        </Flex>

        <Flex
          maxWidth="700px"
          marginBottom="50px"
        >
          <Text
            color="white"
          >
            Geliştirme süreçlerinden sonra yayına alınacak olan Sociapol hakkında daha fazla bilgiye erişebilmek ve takipte kalabilmek için Discord sunucumuza katıl. Discord üzerinde yaptığımız geliştirmelere değiniyor, topluluğumuzu destekliyoruz.
          </Text>
        </Flex>

        <Flex>
          <Button
            bg="white"
            color="#9B59B6"
            borderBottom="solid 8px #EEEEEE"
          >
            <Flex
              alignItems="center"
              justifyContent="center"
            >
              <Flex
                w="34px"
                h="34px"
                bg="white"
                marginRight="15px"
                borderRadius="50%"
                alignItems="center"
                background="#9B59B6"
                justifyContent="center"
              >
                <Image
                  src="/svg/discord.svg"
                  h="16px"
                  w="16px"
                />
              </Flex>

              JOIN DISCORD CHANNEL
            </Flex>
          </Button>
        </Flex>

        <Image
          w="670px"
          h="594px"
          src="/images/car.png"
          position="absolute"
          bottom="-30%"
          right="-17%"
          pointerEvents="none"
        />
      </Flex>
    </Container>
  );
};
