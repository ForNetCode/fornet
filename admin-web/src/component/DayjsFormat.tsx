import dayjs from "dayjs";

export default function DayjsFormat({dateTime}: {dateTime:string}) {
    return <span>{dayjs(dateTime).format('L LT')}</span>
}
