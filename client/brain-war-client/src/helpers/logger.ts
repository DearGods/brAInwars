import pino from "pino";

export const logger = pino({
  name: "brain-wars",
  level: "debug",
});
