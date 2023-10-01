require_relative '../util'

Room = Struct.new(:sections, :sector_id, :checksum)

class Day4
  def initialize
    @input = Util.read_input(4).split("\n").map { |row| parse_room(row) }
  end

  def main
    puts(problem_one.map(&:sector_id).sum)

    puts(problem_two.sort_by(&:first).filter_map do |(decrypted, sector_id)|
      sector_id if decrypted == 'northpole object storage'
    end[0])
  end

  def parse_room(row)
    row.match(/((\w+-)+)(\d+)\[(.*?)\]/) do |m|
      Room.new(m.captures[0].split('-')[..-1], m.captures[2].to_i, m.captures[3])
    end
  end

  def problem_one
    @input.filter do |room|
      most_common = room.sections.join('').split(//).sort.group_by(&:itself).to_a.sort_by { |(k, v)| [-v.length, k] }
      checksum = most_common[..4].map(&:first).join('')
      checksum == room.checksum
    end
  end

  def problem_two
    problem_one.map do |room|
      [room.sections.map do |section|
        section.chars.map { |char| ((char.ord - 97 + (room.sector_id % 26)) % 26 + 97).chr }.join('')
      end.join(' '), room.sector_id]
    end
  end
end

Day4.new.main
