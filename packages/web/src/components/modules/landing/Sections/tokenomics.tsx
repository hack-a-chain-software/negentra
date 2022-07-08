import {
  Grid,
  GridItem,
  Container, Flex, ResponsiveValue,
} from '@chakra-ui/react';
import * as CSS from "csstype";
import { Text, Title } from '@negentra/src/components';

import tokenomics from '@negentra/public/json/tokenomics.json';

const headers = [
  {
    label: 'ROUND',
    cols: 4,
    textAlign: 'left'
  },
  {
    label: '%',
    cols: 2,
    textAlign: 'center'
  },
  {
    label: 'TGE UNLOCK',
    cols: 2,
    textAlign: 'center'
  },
  {
    label: 'CLIFF',
    cols: 2,
    textAlign: 'center'
  },
  {
    label: 'VESTING',
    cols: 2,
    textAlign: 'center'
  },
];

export function Tokenomics () {
  return (
    <Container
      id="tokenomics"
      h="100%"
      w="100vw"
      maxWidth="1214px"
      className="flex items-center flex-col mb-[142px]"
    >
      <Flex
        direction="column"
        alignItems="center"
        marginBottom="68px"
        justifyContent="center"
      >
        <Flex>
          <Title
            textAlign="center"
            textTransform="uppercase"
            className="text-[65px] leading-[1] sm:text-[100px] leading-[114.5px]"
          >
            Tokenomics
          </Title>
        </Flex>

        <Flex>
          <Text
            textAlign="center"
          >
            Sociapol’e üye olduğunda ilk yapacağın şey karakterini yaratmak olacak. Karakterinin giyeceği bütün kıyafet ve aksesuarlar ise birer NFT olarak tasarlandı – dolayısıyla sana ve karakterine ait olacak. Bu NFT’leri satabilir, takas edebilir, yılın belirli dönemlerine özel çıkaracağımız koleksiyonlardan satın alabilirsin. Karakterinin nasıl görüneceği tamamen sana bağlı, ve sana ait!
          </Text>
        </Flex>
      </Flex>

      <Flex
        w="100%"
        maxW="100vw"
        overflow="auto"
        direction="column"
      >
        <Grid
          w="1170px"
          maxW="1170px"
          templateColumns='repeat(12, 1fr)'
          className="bg-[url(/svg/table-header-bg.svg)] bg-[length:1150px_144px] bg-no-repeat pt-[53px] h-[144px] pl-[51px] pr-[68px]"
        >
          {headers.map(({ label, cols, textAlign }, i) => (
            <GridItem
              colSpan={cols}
              key={'neg-tokenomics-header-' + i}
            >
              <Text
                color="white"
                fontSize="22px"
                fontWeight="400"
                fontFamily="Titan One"
                textAlign={textAlign as ResponsiveValue<CSS.Property.TextAlign>}
              >
                { label }
              </Text>
            </GridItem>
          ))}
        </Grid>

        <Flex
          w="1170px"
          minH="564px"
          maxH="564px"
          overflow="auto"
          direction="column"
          boxSizing='border-box'
          className="bg-[url(/svg/table-body-bg.svg)] bg-[length:100%_100%] bg-no-repeat"
        >
          <Flex
            w="1170px"
            maxH="525px"
            overflow="auto"
            direction="column"
            boxSizing='border-box'
            justifyContent="space-between"
            className="pt-[58px] pl-[51px] pr-[68px] space-y-[20px]"
          >
            { tokenomics.map(({ label, percentage, tge, cliff, vesting }, i) => (
              <Grid
                w="100%"
                templateColumns='repeat(12, 1fr)'
                key={'neg-tokenomics-body-' + i}
              >
                <GridItem
                  colSpan={4}
                >
                  <Text
                    color="white"
                    fontSize="22px"
                    fontWeight="700"
                  >
                    { label }
                  </Text>
                </GridItem>

                <GridItem
                  colSpan={2}
                >
                  <Text
                    color="white"
                    fontSize="22px"
                    fontWeight="700"
                    textAlign="center"
                  >
                    { percentage }
                  </Text>
                </GridItem>

                <GridItem
                  colSpan={2}
                >
                  <Text
                    color="white"
                    fontSize="22px"
                    fontWeight="700"
                    textAlign="center"
                  >
                    { tge }
                  </Text>
                </GridItem>

                <GridItem
                  colSpan={2}
                >
                  <Text
                    color="white"
                    fontSize="22px"
                    fontWeight="700"
                    textAlign="center"
                  >
                    { cliff }
                  </Text>
                </GridItem>

                <GridItem
                  colSpan={2}
                >
                  <Text
                    color="white"
                    fontSize="22px"
                    fontWeight="700"
                    textAlign="center"
                  >
                    { vesting }
                  </Text>
                </GridItem>
              </Grid>
            ))}
          </Flex>
        </Flex>
      </Flex>

      <Flex
        w="100%"
        align="center"
        marginTop="35px"
        justifyContent="space-between"
        className="flex-col justify-start items-center space-y-[12px] sm:flex-row sm:justify-between"
      >
        <Flex
          direction="column"
        >
          <Flex
            align="center"
            marginBottom="10px"
          >
            <Flex
              h="14px"
              w="14px"
              borderRadius="100%"
              background="#9B59B6"
              flexShrink="0"
              marginRight="10px"
            />

            <Text
              color="#333333"
              fontWeight="700"
              fontSize="16px"
              whiteSpace="nowrap"
            >
              DLV: Daily Linear Vesting
            </Text>
          </Flex>

          <Flex
            align="center"
          >
            <Flex
              h="14px"
              w="14px"
              borderRadius="100%"
              background="#9B59B6"
              flexShrink="0"
              marginRight="10px"
            />

            <Text
              color="#333333"
              fontWeight="700"
              fontSize="16px"
              whiteSpace="nowrap"
            >
              MV: Montly Vesting
            </Text>
          </Flex>
        </Flex>

        <Flex>
          <Text
            fontSize="16px"
          >
            Total supply of HeC is capped at 100,000,000.
          </Text>
        </Flex>
      </Flex>
    </Container>
  );
};