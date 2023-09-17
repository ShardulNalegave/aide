
import { createBrowserRouter, createRoutesFromElements, Route } from "react-router-dom";
import App from "./App";
import { HomePage } from "./pages/home";
import { ProjectPage } from "./pages/project";

export const router = createBrowserRouter(createRoutesFromElements(
  <Route path='/' element={<App />}>
    <Route path='projects/:projectID' element={<ProjectPage />} loader={async ({ params }) => params.projectID}></Route>
    <Route path='' element={<HomePage />}></Route>
  </Route>
));