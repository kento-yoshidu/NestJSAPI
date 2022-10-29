import { Injectable } from "@nestjs/common"
import { InjectRepository } from "@nestjs/typeorm"
// Repository はデータベースへの参照や作成、更新、削除などのいわゆる CRUD 処理を担います。
import { Repository } from "typeorm"
import { Task } from "./task.entity"
import { CreateTaskDto } from "./dto/create-task-dto"

@Injectable()
export class TasksService {
  constructor (
    @InjectRepository(Task)
    private readonly taskRepository: Repository<Task>
  ) {}

  create(createTaskDto: CreateTaskDto): Promise<Task> {
    return this.taskRepository.save(createTaskDto)
  }
}
