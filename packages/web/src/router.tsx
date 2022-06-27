import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Header } from "./components";
import { Index } from "./pages";
import { routes } from "./routes";

/**
 * @name Router
 * @description - This is the application router that will have all the app routes!
 * And also some business logic to handle near initialization
 */
function Router() {
  return (
    <BrowserRouter>
      <Header />
      <Routes>
        <Route path={routes.home} element={<Index />} />
      </Routes>
    </BrowserRouter>
  );
}

export default Router;
