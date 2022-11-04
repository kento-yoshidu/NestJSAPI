/**
* リクエストを受け取りレスポンスを返す役割を持つControllerを作成
* ControllerはServiceを呼び出すことで、リクエストに応じた適切なレスポンスを返すことができます。
**/

/**
 * Controllerはユーザーのリクエストを受け付けるという役割を持つ。
 * 実際に処理を行うのはService
 */

import { Body, Controller, Delete, Get, Post, Patch, Param } from "@nestjs/common"
import { CreateTaskDto } from "./dto/create-task-dto"
import { UpdateTaskDto } from "./dto/update-task-dto"
import { Task } from "./task.entity"
import { TasksService } from "./task.service"

@Controller("tasks")
export class TasksController {
  constructor(private readonly tasksService: TasksService) {}

  // 「/tasks」という URL に対して POST メソッドが定義されます。
  @Post()
  // Request body にユーザーのリクエストを受け付ける
  create(@Body() createTaskDto: CreateTaskDto): Promise<Task> {
    return this.tasksService.create(createTaskDto)
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

  @Patch(":id")
  update(
    @Param("id") id: string,
    @Body() updateTaskDto: UpdateTaskDto
  ): Promise<Task> {
    return this.tasksService.update(id, updateTaskDto)
  }

  @Delete(":id")
  delete(@Param("id") id: string) {
    return this.tasksService.delete(id)
  }
}
