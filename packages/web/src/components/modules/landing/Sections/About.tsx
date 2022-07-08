import { Tabs, TabList, TabPanels, Tab, TabPanel, Container, Flex, Grid, Image } from '@chakra-ui/react';
import { Text, Title } from '@negentra/src/components';
import items from '@negentra/public/json/about-section.json';

export function About() {
  return (
    <Container
      id="sociapol"
      maxWidth="1280px"
      marginBottom="174px"
      className="flex items-center"
    >
      <Tabs
        align="center"
        variant="unstyled"
        orientation="vertical"
        flexDirection="row-reverse"
        className="xl:w-full mx-auto"
      >
        <TabList
          className="ml-[32px] hidden sm:block my-auto"
        >
          {[...Array(items.length)].map((_, i) => (
            <Flex
              key={'about-tab-list-item' + i}
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
                  fontWeight="800"
                  color="currentColor"
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
              key={'about-tab-panel-item' + i}
            >
              <Grid
                templateColumns="auto 546px"
                className="block xl:grid"
              >
                <Flex
                  direction="column"
                  className="mb-[52px] xl:mb-0"
                >
                  <Flex
                    maxW="510px"
                    marginBottom="26px"
                    className="mx-auto xl:mx-0 xl:max-w-[510px]"
                  >
                    <Title
                      fontSize="80px"
                      lineHeight="94px"
                      className="text-center xl:text-left text-[60px] leading-[1] sm:text-[80px] sm:leading-[94px]"
                    >
                      { title }
                    </Title>
                  </Flex>
      
                  <Flex
                    maxWidth="570px"
                    direction="column"
                    className="space-y-[21px] mx-auto xl:mx-0 text-center xl:text-left"
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
                  w="100%"
                  maxWidth="546px"
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