import { Container, Flex, Image } from '@chakra-ui/react';
import { Button3d, Text, Title } from '@negentra/src/components';

export function FollowUp() {
  return (
    <Container
      maxWidth="1410px"
      marginBottom="200px"
      className="flex items-center flex-col mb-[142px] px-0"
    >
      <Flex
        w="100%"
        height="640px"
        maxW="1091px"
        direction="column"
        position="relative"
        className="bg-[linear-gradient(110deg,_#FF6F00_60%,_#9B59B6_60%)] px-[18px] items-center justify-center lg:px-0 lg:items-start lg:justify-start lg:bg-[url(/svg/follow-up-bg.svg)] lg:bg-[center_top_-88px] lg:pt-[101px] lg:pl-[112px]"
      >
        <Flex
          maxWidth="600px"
          marginBottom="20px"
          className="text-center lg:text-left"
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
          className="text-center lg:text-left"
        >
          <Text
            color="white"
          >
            Geliştirme süreçlerinden sonra yayına alınacak olan Sociapol hakkında daha fazla bilgiye erişebilmek ve takipte kalabilmek için Discord sunucumuza katıl. Discord üzerinde yaptığımız geliştirmelere değiniyor, topluluğumuzu destekliyoruz.
          </Text>
        </Flex>

        <Flex>
          <Button3d
            bg="#EEEEEE"
            color="white"
          >
            <Flex
              alignItems="center"
              justifyContent="center"
              color="#9B59B6"
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

              <Text
                fontSize="18px"
                color="#9B59B6"
                fontFamily="Titan One"
              >
                JOIN DISCORD CHANNEL
              </Text>
            </Flex>
          </Button3d>
        </Flex>

        <Image
          w="670px"
          h="594px"
          src="/images/car.png"
          position="absolute"
          bottom="-30%"
          right="-17%"
          pointerEvents="none"
          className="hidden xl:block"
        />
      </Flex>
    </Container>
  );
};
