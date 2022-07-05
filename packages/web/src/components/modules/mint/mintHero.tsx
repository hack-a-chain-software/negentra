import { useState } from 'react';
import { Button, Title, Text } from '@negentra/src/components';
import { Container, Grid, Flex, Image } from '@chakra-ui/react';

export function MintHero() {
  const [soldOut, setSoldOut] = useState(false);

  return (
    <Container
      w="100vw"
      minH="100vh"
      maxWidth="1920px"
      className="bg-[url('/images/hero-bg.png')] bg-[length:auto_auto] bg-[left_-20px_top_42px] bg-no-repeat"
    >
      <Container
        h="100%"
        maxWidth="1410px"
        className="flex items-center pt-[180px]"
      >
        <Grid
          w="100%"
          paddingTop="72px"
          templateColumns="578px auto"
          justifyContent="space-between"
          className="block xl:grid"
        >
          <Flex
            direction="column"
            justifyContent="center"
          >
            <Flex
              direction="column"
              className="mb-[33px] xl:66px"
            >
              <Text
                fontSize="22px"
                fontWeight="600"
                marginBottom="-8px"
                className="text-center md:text-left"
              >
                FUNNIES, PARTIES, FRIENDS.
              </Text>

              <Title
                marginBottom="25px"
                maxWidth="578px"
                className="text-[80px] leading-[1] mt-[10px] text-center sm:text-[100px] sm:leading-[100px] sm:mt-0 md:text-left mx-auto md:mx-0"
              >
                MINT <br className="hidden md:block" /> NOW!
              </Title>

              <Text
                maxWidth="669px"
                fontSize="24px"
                marginBottom="32px"
                className="text-center md:text-left"
              >
                Get one of the 10.000 unique sociapol characters, Mint yours now.
              </Text>


              <Text
                fontSize="36px"
                letterSpacing="-3px"
                fontFamily="Titan One"
                className="text-center md:text-left"
              >
                {
                  soldOut 
                  ? ( 'SOLD OUT!' ) 
                  : ( '0 / 10.000' )
                }
              </Text>
            </Flex>

            <Flex
              className='justify-center md:justify-start'
            >
              <Button
                flex="1"
                maxWidth="273px"
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
                    justifyContent="center"
                  >
                    <Image
                      src="/svg/market.svg"
                      h="16px"
                      w="16px"
                    />
                  </Flex>

                  Mint Your NFT
                </Flex>
              </Button>
            </Flex>
          </Flex>

          <Flex
            align="center"
            justifyContent="center"
            className="hidden xl:flex"
            maxWidth="690px"
          >
            <Image
              src="/images/hero.png"
              w="full"
              h="full"
            />
          </Flex>
        </Grid>
      </Container>
    </Container>
  );
}
