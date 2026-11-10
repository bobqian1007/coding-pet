import "./Pet.css";

export type PetStatus = "IDLE" | "COMPLETED";

interface PetProps {
  status: PetStatus;
}

const statusText: Record<PetStatus, string> = {
  IDLE: "等待 Codex",
  COMPLETED: "本轮完成",
};

const statusDetail: Record<PetStatus, string> = {
  IDLE: "Codex 完成后，我会来提醒你",
  COMPLETED: "可以回来查看本轮结果了",
};

export function Pet({ status }: PetProps) {
  return (
    <section className={`pet-card pet-card--${status.toLowerCase()}`}>
      <div className="pet" role="img" aria-label={`Coding Pet：${statusText[status]}`}>
        <span className="pet__ear pet__ear--left" />
        <span className="pet__ear pet__ear--right" />
        <span className="pet__face">
          <span className="pet__eye pet__eye--left" />
          <span className="pet__eye pet__eye--right" />
          <span className="pet__mouth" />
        </span>
        <span className="pet__body" />
        <span className="pet__tail" />
      </div>
      <div className="pet-card__copy">
        <strong>{statusText[status]}</strong>
        <span>{statusDetail[status]}</span>
      </div>
    </section>
  );
}
