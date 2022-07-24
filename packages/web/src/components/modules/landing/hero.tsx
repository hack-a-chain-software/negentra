import { motion } from 'framer-motion';
import { useNavigate } from 'react-router';
import { Container, Grid, Flex, Image } from '@chakra-ui/react';
import { Button3d, Button, Title, Text } from '@negentra/src/components';

export function Hero() {
  const navigate = useNavigate();

  return (
    <motion.div
      animate={{ opacity: 1 }}
      initial={{ opacity: 0 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.55 }}
    >
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
                <Button3d
                  width="273px"
                  onClick={() => navigate('/mint')}
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
                      background="white"
                      justifyContent="center"
                    >
                      <Image
                        h="16px"
                        w="16px"
                        src="/svg/market.svg"
                      />
                    </Flex>
      
                    <Text
                      fontSize="18px"
                      color="white"
                      fontFamily="Titan One"
                      textShadow="0px 2px 0px #AAAAAA"
                    >
                      Mint Character's
                    </Text>
                  </Flex>
                </Button3d>

                <Button
                  flex="1"
                  bg="white"
                  color="#333333"
                  height="77px"
                  maxWidth="273px"
                  border="1px solid #979797"
                  position="relative"
                  top="-3px"
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
    </motion.div>
  );
}
