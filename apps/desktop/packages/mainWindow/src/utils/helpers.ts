export const parseTwoDigitNumber = (number: number) => {
  return number.toString().length === 1 ? `0${number}` : number;
};