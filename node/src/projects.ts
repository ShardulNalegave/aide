
import { Router } from 'express';

import { ProjectModel } from './models/project.js';
import { TaskModel } from './models/task.js';

export const projectsRoutes = Router();

projectsRoutes.get('/', async (_, res) => {
  let projects = await ProjectModel.find({});
  res.json(projects);
});

projectsRoutes.post('/', async (req, res) => {
  let name = req.body.name;
  if (!name) res.status(400).send('Invalid data');

  name = name.trim();
  if (name === '') res.status(400).send('Invalid data');

  let project = new ProjectModel({
    name,
  });
  await project.save();

  res.json(project.toJSON());
});

projectsRoutes.get('/:projectID/tasks', async (req, res) => {
  let projectID = req.params.projectID;
  let tasks = await TaskModel.find({ project: projectID });
  res.json(tasks);
});

projectsRoutes.get('/:projectID', async (req, res) => {
  let projectID = req.params.projectID;
  let project = await ProjectModel.findOne({ _id: projectID });
  res.json(project);
});

projectsRoutes.delete('/:projectID', async (req, res) => {
  let projectID = req.params.projectID;
  try {
    await ProjectModel.deleteOne({ _id: projectID });
    res.status(200).send('Done');
  } catch (_) {
    res.status(400).send('Invalid data');
  }
});

projectsRoutes.put('/:projectID', async (req, res) => {
  let projectID = req.params.projectID;
  let name = req.body.name;

  try {
    let project = await ProjectModel.findOne({ _id: projectID });
    await project.updateOne({
      name,
    });
    res.send('Done');
  } catch (_) {
    res.status(500).send('Couldn\'t Update');
  }
});