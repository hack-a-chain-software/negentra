import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Header, Footer } from "./components";
import { Index } from "./pages";
import { Mint } from './pages/mint';
import { routes } from "./routes";

function Router() {
  return (
    <BrowserRouter>
      <Header />
      <Routes>
        <Route path={routes.home} element={<Index />} />
        <Route path={routes.mint} element={<Mint />} />
      </Routes>
      <Footer/>
    </BrowserRouter>
  );
}

export default Router;
