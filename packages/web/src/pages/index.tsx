import { 
  Hero, 
  Play,
  Team,
  About,
  Toys,
  Roadmap,
  Advisors,
  FollowUp,
  Partners,
  Tokenomics,
  Trailer,
} from '../components';

export function Index() {
  return (
    <main>
      <Hero/>
      <Trailer/>
      <About/>
      <Play/>
      <Toys/>
      <Tokenomics/>
      <Roadmap/>
      <Team/>
      <Advisors/>
      <Partners/>
      <FollowUp/>
    </main>
  );
}
