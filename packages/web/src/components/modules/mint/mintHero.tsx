import { useState } from 'react';
import { motion } from 'framer-motion';
import { useContract } from '@negentra/src/stores/contract';
import { Title, Text, RadioCard, Button3d, If } from '@negentra/src/components';
import { Container, Grid, Flex, Image, useRadioGroup } from '@chakra-ui/react';

export function MintHero() {
  const [ type, setType ] = useState('Male');

  const options = ['Male', 'Female']

  const { getRootProps, getRadioProps } = useRadioGroup({
    name: 'char',
    defaultValue: type,
    onChange: (value) => {
      setType(value)
    },
  })

  const group = getRootProps();

  const {
    mintedChar,
  } = useContract();

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
                className="mb-[8px] xl:66px"
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
              </Flex>
              
              <If
                condition={!mintedChar}
              >
                <Flex>
                  <Text
                    maxWidth="669px"
                    fontSize="22px"
                    marginBottom="18px"
                    className="text-center md:text-left"
                  >
                    Select your character's gender
                  </Text>
                </Flex>

                <Flex
                  className='justify-center md:justify-start flex-col space-y-[32px]'
                >
                  <Flex
                    w="100%"
                  >
                    <Flex 
                      {...group}
                      className="space-x-[12px]"
                    >
                      {options.map((value) => {
                        const radio = getRadioProps({ value })

                        return (
                          <RadioCard key={value} {...radio}>
                            <Flex
                              outline="none"
                              alignItems="center"
                              justifyContent="start"
                              height="52px"
                            >
                              <Flex
                                alignItems="center"
                                justifyContent="center"
                                marginRight="12px"
                              >
                                <Image
                                  h="40px"
                                  w="auto"
                                  src={ value === 'Male' ? '/images/maleHead.png' : '/images/femaleHead.png' }
                                />
                              </Flex>
                              
                              <Text
                                color="white"
                                fontFamily="Titan One"
                              >
                                { value }
                              </Text>
                            </Flex>
                          </RadioCard>
                        )
                      })}
                    </Flex>
                  </Flex>

                  <Flex
                    flexDirection="column"
                  >
                    <Text
                      maxWidth="669px"
                      fontSize="22px"
                      marginBottom="18px"
                      className="text-center md:text-left"
                    >
                      Mint to play the game!
                    </Text>

                    <Button3d
                      width="352px"
                      onClick={() => mint(type)}
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

                        <Text
                          color="white"
                          fontSize="18px"
                          fontFamily="Titan One"
                        >
                          Mint and play!
                        </Text>
                      </Flex>
                    </Button3d>
                  </Flex>
                </Flex>
              </If>

              <If
                condition={mintedChar}
              >
                <Flex
                  height="80px"
                  alignItems="center"
                >
                  <Text
                    color="red"
                  >
                    Ooops! You already minted a character
                  </Text>
                </Flex>
              </If>
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
