import type {
  PropagationContext,
  PropagationRun,
  PropagationStrategy,
} from "./simulation";

export function floodingStrategy(
  maxBackoffSlots: number,
): PropagationStrategy {
  if (!Number.isSafeInteger(maxBackoffSlots) || maxBackoffSlots < 0) {
    throw new RangeError("Maximum backoff must be a non-negative integer.");
  }

  return {
    name: "Flood",
    createRun(): PropagationRun {
      const forwarded = new Set<string>();
      const maybeForward = (
        nodeId: string,
        context: PropagationContext,
      ): readonly number[] => {
        if (forwarded.has(nodeId)) return [];
        forwarded.add(nodeId);
        return [1 + Math.floor(context.random() * (maxBackoffSlots + 1))];
      };

      return {
        onStart(context) {
          forwarded.add(context.sourceId);
          return [0];
        },
        onReception(reception, context) {
          return maybeForward(reception.receiverId, context);
        },
      };
    },
  };
}
