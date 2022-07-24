import { Container, Flex, Grid } from '@chakra-ui/react';
import advisors from '@negentra/public/json/team.json';
import { PersonCard, Title, Text } from '@negentra/src/components';

export function Advisors() {
  return (
    <Container
      id="advisors"
      maxWidth="1500px"
      marginBottom="125px"
      className="flex flex-col items-center"
    >
      <Flex
        marginBottom="12px"
      >
        <Title
          textTransform="uppercase"
          className="text-[80px] leading-[1] sm:text-[100px] leading-[114.5px]"
        >
          Advisors
        </Title>
      </Flex>

      <Flex
        marginBottom="50px"
      >
        <Text
          maxWidth="1184px"
          textAlign="center"
        >
          Sociapol’e üye olduğunda ilk yapacağın şey karakterini yaratmak olacak. Karakterinin giyeceği bütün kıyafet ve aksesuarlar ise birer NFT olarak tasarlandı – dolayısıyla sana ve karakterine ait olacak. Bu NFT’leri satabilir, takas edebilir, yılın belirli dönemlerine özel çıkaracağımız koleksiyonlardan satın alabilirsin. Karakterinin nasıl görüneceği tamamen sana bağlı, ve sana ait!
        </Text>
      </Flex>

      <Grid
        width="100%"
        rowGap="30px"
        justifyContent="center"
        className="grid-cols-[repeat(auto-fill,242px)]"
      >
        { advisors && advisors.map((person: object, i: number) => (
          <PersonCard
            { ...person }
            key={'neg-advisors-card' + i}
          />
        ))}
      </Grid>
    </Container>
  );
};