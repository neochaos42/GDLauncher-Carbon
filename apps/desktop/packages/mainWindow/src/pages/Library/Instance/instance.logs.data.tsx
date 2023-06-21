import { rspc } from "@/utils/rspcClient";

//@ts-ignore
const fetchData = ({ params }) => {
  const logs = rspc.createQuery(() => ["instance.getLogs"]);

  const instanceDetails = rspc.createQuery(() => [
    "instance.getInstanceDetails",
    parseInt(params.id, 10),
  ]);

  return { logs, instanceDetails };
};

export default fetchData;