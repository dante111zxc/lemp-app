/* eslint-disable @typescript-eslint/no-extraneous-class */
export interface EnumOption {
  value: string | number
  label: string
}

export abstract class BaseEnum {
  protected constructor() {}
  protected static readonly values: Array<{
    label: string
    value: string | number
  }> = []

  /**
   * Get label by value
   * @param value - Enum value
   * @returns Label of the enum or undefined if not found
   */
  static getLabel(value: string | number): string | undefined {
    return this.values.find(v => v.value === value)?.label
  }

  /**
   * Check if value matches the enum value
   * @param value - Value to check
   * @param enumValue - Enum value to compare
   * @returns True if values match
   */
  static is(value: string | number, enumValue: string | number): boolean {
    return value === enumValue
  }

  /**
   * Check if value does NOT match the enum value
   * @param value - Value to check
   * @param enumValue - Enum value to compare
   * @returns True if values don't match
   */
  static isNot(value: string | number, enumValue: string | number): boolean {
    return value !== enumValue
  }

  /**
   * Check if value exists in enum
   * @param value - Value to check
   * @returns True if value exists in enum
   */
  static hasValue(value: string | number): boolean {
    return this.values.some(v => v.value === value)
  }

  /**
   * Get all enum options as array
   * @returns Array of enum options with value and label
   */
  static list(): EnumOption[] {
    return this.values
  }

  /**
   * Get all enum values
   * @returns Array of all enum values
   */
  static getValues(): (string | number)[] {
    return this.values.map(v => v.value)
  }

  /**
   * Get all enum labels
   * @returns Array of all enum labels
   */
  static getLabels(): string[] {
    return this.values.map(v => v.label)
  }

  /**
   * Get enum as key-value object
   * @returns Object with enum values and labels
   */
  static toObject(): Record<string | number, string> {
    return this.values.reduce(
      (acc, item) => ({ ...acc, [item.value]: item.label }),
      {} as Record<string | number, string>
    )
  }
}
