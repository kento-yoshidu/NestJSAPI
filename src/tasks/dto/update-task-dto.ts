import { IsNotEmpty, IsString, IsBoolean } from "class-validator"

export class UpdateTaskDto {
  @IsNotEmpty()
  @IsString()
  name: string

  @IsNotEmpty()
  @IsString()
  isCompleted: boolean
}
