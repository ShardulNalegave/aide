
import { Container, ScrollArea, Title } from "@mantine/core";
import { NavLink, useLoaderData } from "react-router-dom";
import { useGET } from "../hooks/loadData";
import { IProject } from "../models/project";
import { ITask } from "../models/task";
import { TaskList } from "../components/taskList";

export function ProjectPage() {
  const projectID = useLoaderData();
  const { data: projectData, loaded } = useGET<IProject | undefined>(`http://localhost:5000/projects/${projectID}`, undefined);
  const { data: projectTasks, loaded: loadedProjectTasks } = useGET<ITask[]>(`http://localhost:5000/projects/${projectID}/tasks`, []);

  return (
    !loaded || !loadedProjectTasks ? <></> :
      <ScrollArea style={{ background: '#141517', color: '#EFEFEF', height: '100vh' }}>
        <Container style={{ paddingTop: '30px' }}>
          <NavLink to='/'>Home</NavLink>
          <div style={{ height: '25px' }}></div>
          <Title>{projectData?.name}</Title>
          <div style={{ height: '15px' }}></div>
          <TaskList tasks={projectTasks} />
        </Container>
      </ScrollArea>
  );
}