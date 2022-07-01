import { Container, Flex, Grid } from '@chakra-ui/react';
import { PersonCard, Text, Title } from '@negentra/src/components';

import team from '@negentra/public/json/team.json';

export function Team() {
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
        <Title
          textTransform="uppercase"
        >
          Our Team
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
        templateColumns='repeat(6, 1fr)'
        rowGap="30px"
      >
        {team && team.map((person, i) => (
          <PersonCard
            { ...person }
            key={'neg-team-card' + i}
          />
        ))}
      </Grid>
    </Container>
  );
};