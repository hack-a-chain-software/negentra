import { Button, Title, Text } from '@negentra/src/components';
import { Container, Grid, Flex, Image } from '@chakra-ui/react';

export function Hero() {
  return (
    <Container
      w="100vw"
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
                className="text-[80px] leading-[1] mt-[10px] text-center sm:text-[100px] sm:leading-[100px] sm:mt-0 md:text-left"
              >
                PLAY {'&'} COLLECT!
              </Title>

              <Text
                maxWidth="669px"
                className="text-center md:text-left"
              >
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Mauris rhoncus nisi a quam accumsan dapibus. Curabitur turpis massa, pretium id risus sed, gravida tincidunt tellus.
              </Text>
            </Flex>

            <Flex
              className='sm:space-x-[30px] space-y-[12px] flex-col items-center sm:flex-row sm:space-y-0'
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

                  NFT Marketplace
                </Flex>
              </Button>

              <Button
                flex="1"
                bg="white"
                color="#333333"
                maxWidth="273px"
                border="1px solid #979797"
                _active={{ background: '#333333', color: 'white' }}
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
