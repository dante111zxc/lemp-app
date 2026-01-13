import { BaseEnum } from './BaseEnum'

export class EnumServiceStatus extends BaseEnum {
  static readonly ACTIVE = 1
  static readonly STOPPED = 2
  static readonly ERROR = 3
  static readonly STARTING = 4
  static readonly STOPPING = 5
  static readonly NOT_INSTALLED = 6
  static readonly NOT_ACTIVE = 7

  protected static override values = [
    { value: EnumServiceStatus.ACTIVE, label: 'Active' },
    { value: EnumServiceStatus.STOPPED, label: 'Stopped' },
    { value: EnumServiceStatus.ERROR, label: 'Error' },
    { value: EnumServiceStatus.STARTING, label: 'Starting' },
    { value: EnumServiceStatus.STOPPING, label: 'Stopping' },
    { value: EnumServiceStatus.NOT_INSTALLED, label: 'Not Installed' },
    { value: EnumServiceStatus.NOT_ACTIVE, label: 'Not Active' },
  ]
}
