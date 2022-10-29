/**
* TasksModule は、先ほど作成した Task Entity やこれから作成するコントローラやサービスをひとまとまりにするものです。
* 後ほど、この Tasks Module をアプリケーションのルートモジュールである App Module に渡します。
**/

import { Module } from "@nestjs/common";
import { TypeOrmModule } from "@nestjs/typeorm"
import { Task } from "./task.entity"

@Module({
  imports: [TypeOrmModule.forFeature([Task])],
  providers: [],
  controllers: []
})

export class TasksModule {}
