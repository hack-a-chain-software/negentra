import { Button } from '../../shared';
import { Container, Grid, Flex, Image, Text } from '@chakra-ui/react';

export function Hero() {
  return (
    <Container
      w="100vw"
      minHeight="100vh"
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
          templateColumns="auto 690px"
          paddingTop="72px"
          gap="112px"
        >
          <Flex
            direction="column"
            justifyContent="center"
          >
            <Flex
              direction="column"
              marginBottom="66px"
            >
              <Text
                color="#333333"
                fontSize="22px"
                fontWeight="medium"
                marginBottom="-16px"
              >
                FUNNIES, PARTIES, FRIENDS.
              </Text>

              <Text
                fontSize="100px"
                lineHeight="117px"
                color="#333333"
                marginBottom="28px"
              >
                PLAY {'&'} COLLECT!
              </Text>

              <Text
                fontSize="18px"
                color="#333333"
                maxWidth="669px"
              >
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris rhoncus nisi a quam accumsan dapibus. Curabitur turpis massa, pretium id risus sed, gravida tincidunt tellus.
              </Text>
            </Flex>

            <Flex
              className='space-x-[30px]'
            >
              <Button
                flex="1"
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

                  NFT Marketplace
                </Flex>
              </Button>

              <Button
                flex="1"
                bg="white"
                color="#333333"
                border="1px solid #979797"
              >
                <Flex
                  justifyContent="start"
                >
                  Look At Whitepaper
                </Flex>
              </Button>
            </Flex>
          </Flex>

          <Flex
            w="full"
            align="center"
            justifyContent="center"
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
