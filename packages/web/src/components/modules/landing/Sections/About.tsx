import { Tabs, TabList, TabPanels, Tab, TabPanel, Container, Flex, Grid, Image } from '@chakra-ui/react';
import { Text, Title } from '@negentra/src/components';
import items from '@negentra/public/json/about-section.json';

export function About() {
  return (
    <Container
      h="100%"
      w="100vw"
      maxWidth="1410px"
      marginBottom="174px"
      className="flex items-center"
    >
      <Tabs
        w="100%"
        align="center"
        variant="unstyled"
        orientation="vertical"
        flexDirection="row-reverse"
      >
        <TabList>
          {[...Array(items.length)].map((_, i) => (
            <Flex
              key={i}
              align="start"
              position="relative"
              direction="column"
              alignItems="center"
              justifyContent="center"
            >
              { i > 0 && 
                <Flex
                  h="46px"
                  w="4px"
                  marginY="13px"
                  flexShrink="0"
                  background="#9959B4"
                  borderRadius="1.5px"
                /> 
              }
              
              <Tab
                w="64px"
                h="64px"
                bg="white"
                flexShrink="0"
                borderRadius="50%"
                border="1px solid #9959B4"
                color="#9959B4"
                _selected={{ bg: '#9959B4', borderBottom: 'solid 4px #71368A', color: 'white' }}
              >
                <Text
                  fontSize="18px"
                  fontWeight="500"
                >
                  { i + 1 }
                </Text>
              </Tab>
            </Flex>
          ))}
        </TabList>

        <TabPanels>
          {items.map(({ title, description, image }, i) => (
            <TabPanel
              key={i}
            >
              <Grid
                templateColumns="auto 546px"
              >
                <Flex
                  direction="column"
                >
                  <Flex
                    maxW="450px"
                    marginBottom="26px"
                  >
                    <Title
                      fontSize="80px"
                      lineHeight="94px"
                      textAlign="start"
                    >
                      { title }
                    </Title>
                  </Flex>
      
                  <Flex
                    maxWidth="570px"
                    textAlign="start"
                    direction="column"
                    className="space-y-[21px]"
                  >
                    { description?.map((text: string, i: number) => (
                      <Text
                        key={'neg-description' + i}
                      > 
                      { text }
                      </Text>
                    ))}
                  </Flex>
                </Flex>

                <Image
                  src={image}
                />
              </Grid>
            </TabPanel>
          ))}
        </TabPanels>
      </Tabs>
    </Container>
  );
};