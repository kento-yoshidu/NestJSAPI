/**
* リクエストを受け取りレスポンスを返す役割を持つControllerを作成
* ControllerはServiceを呼び出すことで、リクエストに応じた適切なレスポンスを返すことができます。
**/

import { Body, Controller, Get, Post, Param } from "@nestjs/common"
import { CreateTaskDto } from "./dto/create-task-dto"
import { Task } from "./task.entity"
import { TasksService } from "./task.service"

@Controller("tasks")
export class TasksController {
  constructor(private readonly tasksService: TasksService) {}

  // 「/tasks」という URL に対して POST メソッドが定義されます。
  @Post()
  // Request body にユーザーのリクエストを受け付ける
  create(@Body() createTaskDto: CreateTaskDto): Promise<Task> {
    return this.tasksService.create(CreateTaskDto)
  }

  /* Task を取得するためのリクエストを受け付けるメソッド */

  @Get()
  findAll(): Promise<Task[]> {
    return this.tasksService.findAll()
  }

  @Get(":id")
  findOne(@Param("id") id: string): Promise<Task> {
    return this.tasksService.findOne(id)
  }
}