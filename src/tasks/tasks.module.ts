/**
* TasksModule は、先ほど作成した Task Entity やこれから作成するコントローラやサービスをひとまとまりにするものです。
* Controller・Service・Entity とまとめて TasksModule とします。
* この Tasks Module をアプリケーションのルートモジュールである App Module に渡します。
**/

import { Module } from "@nestjs/common"
import { TypeOrmModule } from "@nestjs/typeorm"
import { Task } from "./task.entity"
import { TasksController } from "./tasks.controller"
import { TasksService } from "./task.service"

@Module({
  imports: [TypeOrmModule.forFeature([Task])],
  providers: [TasksController],
  controllers: [TasksService]
})

export class TasksModule {}
