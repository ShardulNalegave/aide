
import { Card, Text, Title } from "@mantine/core";
import { ITask } from "../models/task";

interface TaskListProps {
  tasks: ITask[],
}

export function TaskList({ tasks } : TaskListProps) {
  return (
    <div>
      {tasks.map(task => <Task key={task._id} data={task} />)}
    </div>
  );
}

interface TaskProps {
  data: ITask,
}

export function Task({ data } : TaskProps) {
  return (
    <Card style={{ margin: '5px' }}>
      <Title order={3}>{data.title}</Title>
      <div style={{ height: '5px' }}></div>
      <Text >{data.description}</Text>
    </Card>
  );
}
