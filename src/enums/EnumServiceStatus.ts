import { BaseEnum } from './BaseEnum'

export class EnumServiceStatus extends BaseEnum {
  static readonly NOT_INSTALLED = 0
  static readonly RUNNING = 1
  static readonly STOPPED = 2

  protected static override values = [
    { value: EnumServiceStatus.NOT_INSTALLED, label: 'Not Installed' },
    { value: EnumServiceStatus.RUNNING, label: 'Active' },
    { value: EnumServiceStatus.STOPPED, label: 'Stopped' },
  ]
}
