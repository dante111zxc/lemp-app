import { BaseEnum } from './BaseEnum'

export class EnumServiceStatus extends BaseEnum {
  static readonly STOPPED = 0
  static readonly RUNNING = 1
  static readonly NOT_INSTALLED = 2

  protected static override values = [
    { value: EnumServiceStatus.RUNNING, label: 'Active' },
    { value: EnumServiceStatus.STOPPED, label: 'Stopped' },
    { value: EnumServiceStatus.NOT_INSTALLED, label: 'Not Installed' },
  ]
}
