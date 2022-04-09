// import
import init, {parse_cron_cal} from '../../pkg/cron_cal_wasm.js'
import ApexCharts from 'apexcharts'
import flatpickr from 'flatpickr'
import '../css/style.css'

// function
export function run() {
    init()
        .then(() => {
            var start = getUtcTimestampFromYmd(startDate.element.value)
            var end = getUtcTimestampFromYmd(endDate.element.value)
            if (start >= end) {
                alert('`end` MUST be  greater than `start`')
                return
            }

            var days = Number(Math.floor((end - start) / 86400000))
            var d
            try {
                var input = document.getElementById('input').value
                d = Object.values(parse_cron_cal(input, BigInt(start / 1000), days))
            } catch(e) {
                alert(e)
                return
            }

            if (d == null || d.length == 0) {
                return 
            }

            var data = arrayToChunks(d, 2).map(p => {
                var prop = {}
                prop.x = 'cron schedule'
                prop.y = [
                    Number(p[0] * 1000n),
                    Number(p[1] * 1000n)
                ]

                return prop
            })
            plot(data, start, days)
        })
}

function getUtcTimestampFromYmd(ymd) {
    var date = flatpickr.parseDate(ymd, 'Y-m-d')

    return new Date(Date.UTC(date.getFullYear(), date.getMonth(), date.getDate(), 0, 0, 0)).getTime()
}

function arrayToChunks(arr, size) {
    var res = []
    for (var i = 0; i < arr.length; i += size) {
        var chunk = arr.slice(i, i + size)
        res.push(chunk)
    }

    return res
}

function plot(data, date, days) {
    var min = date
    var max = date + 86400000 * days

    var options = {
        series: [
            {
                data: data
            }
        ],
        tooltip: {
            x: {
                format: 'MM/dd HH:mm'
            }
        },
        chart: {
            height: 350,
            type: 'rangeBar',
            zoom: {
                enabled: false
            }
        },
        plotOptions: {
            bar: {
                horizontal: true
            }
        },
        xaxis: {
            type: 'datetime',
            min: min,
            max: max
        }
    }

    var chart = new ApexCharts(document.getElementById('chart'), options)
    if (document.getElementById('chart').innerHTML.length > 0) {
        document.getElementById('chart').innerHTML = ''
        chart.render()
    } else {
        chart.render()
    }
}

export function clear() {
    document.getElementById('input').value = ''
    document.getElementById('chart').innerHTML = ''
}

// init
const startDate = flatpickr('#start', { defaultDate: new Date() })
const endDate = flatpickr('#end', { defaultDate: new Date().fp_incr(1) })

var textareas = document.getElementsByTagName('textarea')
Array.prototype.forEach.call(textareas, function(elem) {
    elem.placeholder = elem.placeholder.replace(/\\n/g, '\n')
})

document.getElementById('run').onclick = run
document.getElementById('clear').onclick = clear
