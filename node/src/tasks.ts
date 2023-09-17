
import { Router } from 'express';

import { TaskModel } from './models/task.js';

export const tasksRoutes = Router();

tasksRoutes.get('/', async (_, res) => {
  let tasks = await TaskModel.find({});
  res.json(tasks);
});

tasksRoutes.post('/', async (req, res) => {
  let title = req.body.title;
  let description = req.body.description;
  let project = req.body.project;
  if (!title || !description || !project) res.status(400).send('Invalid data');

  title = title.trim();
  description = description.trim();
  project = project.trim();
  if (title === '' || project === '') res.status(400).send('Invalid data');

  let task = new TaskModel({
    title, description, project,
    completed: false,
  });
  await task.save();

  res.send(task.toJSON());
});

tasksRoutes.get('/:taskID', async (req, res) => {
  let taskID = req.params.taskID;
  let task = await TaskModel.findOne({ _id: taskID });
  res.json(task);
});

tasksRoutes.delete('/:taskID', async (req, res) => {
  let taskID = req.params.taskID;
  try {
    await TaskModel.deleteOne({ _id: taskID });
    res.status(200).send('Done');
  } catch (_) {
    res.status(400).send('Invalid data');
  }
});

tasksRoutes.put('/:taskID', async (req, res) => {
  let taskID = req.params.taskID;
  let title = req.body.title;
  let description = req.body.description;
  let project = req.body.project;
  let completed = req.body.completed;

  try {
    let task = await TaskModel.findOne({ _id: taskID });
    await task.updateOne({
      title, description, project, completed,
    });
    res.send('Done');
  } catch (_) {
    res.status(500).send('Couldn\'t update');
  }
});