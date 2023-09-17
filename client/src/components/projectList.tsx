
import { Card, Title } from "@mantine/core";
import { IProject } from "../models/project";
import { useNavigate } from "react-router-dom";

interface ProjectListProps {
  projects: IProject[],
}

export function ProjectList({ projects } : ProjectListProps) {
  return (
    <div>
      {projects.map(project => <Project key={project._id} data={project} />)}
    </div>
  );
}

interface ProjectProps {
  data: IProject,
}

export function Project({ data } : ProjectProps) {
  const navigate = useNavigate();

  return (
    <Card style={{ margin: '5px', cursor: 'pointer' }} onClick={() => navigate(`/projects/${data._id}`)}>
      <Title order={3}>{data.name}</Title>
    </Card>
  );
}