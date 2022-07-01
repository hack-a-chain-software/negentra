import { Container, Flex, Grid } from '@chakra-ui/react';
import advisors from '@negentra/public/json/team.json';
import { PersonCard, Title, Text } from '@negentra/src/components';

export function Advisors() {
  return (
    <Container
      h="100%"
      w="100vw"
      maxWidth="1410px"
      marginBottom="125px"
      className="flex flex-col items-center"
    >
      <Flex
        marginBottom="12px"
      >
        <Title>
          ADVISORS
        </Title>
      </Flex>

      <Flex
        marginBottom="50px"
      >
        <Text
          textAlign="center"
        >
          Sociapol’e üye olduğunda ilk yapacağın şey karakterini yaratmak olacak. Karakterinin giyeceği bütün kıyafet ve aksesuarlar ise birer NFT olarak tasarlandı – dolayısıyla sana ve karakterine ait olacak. Bu NFT’leri satabilir, takas edebilir, yılın belirli dönemlerine özel çıkaracağımız koleksiyonlardan satın alabilirsin. Karakterinin nasıl görüneceği tamamen sana bağlı, ve sana ait!
        </Text>
      </Flex>

      <Grid
        rowGap="30px"
        templateColumns='repeat(6, 1fr)'
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